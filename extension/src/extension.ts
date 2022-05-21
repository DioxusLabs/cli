import * as vscode from 'vscode';
import { spawn } from "child_process";

export function activate(context: vscode.ExtensionContext) {
	function registerCommand(cmd: string) {
		function convert(cmd: string) {
			const editor = vscode.window.activeTextEditor;// Get the active text editor
			if (editor) {
				const html = editor.document.getText(editor.selection);
				if (html.length > 0) {
					let params = ["translate"];
					if (cmd.includes("Component")) params.push("--component");
					params.push("--source");
					params.push(html);
					const child_proc = spawn("dioxus", params);
					let result = '';
					child_proc.stdout?.on('data', data => result += data);
					child_proc.on('close', () => {
						if (result.length > 0) editor.edit(editBuilder => editBuilder.replace(editor.selection, result));
					});
				} else {
					vscode.window.showWarningMessage("Please select HTML fragment before invoking this command!");
				}
			}
		}
		const handle = vscode.commands.registerCommand(cmd, () => convert(cmd));
		context.subscriptions.push(handle);
	}

	registerCommand('extension.htmlToDioxusRsx');
	registerCommand('extension.htmlToDioxusComponent');

	// the current rsx preview
	let currentPanel: vscode.WebviewPanel | undefined = undefined;
	let preview_selection: vscode.Selection | undefined = undefined;
	function html_from_rsx(rsx: string, callback: { (html: string): void }) {
		let params = ["render"];
		params.push("--source");
		params.push(rsx);
		const child_proc = spawn("dioxus", params);
		let result = '';
		child_proc.stdout?.on('data', data => result += data);
		child_proc.on('close', () => callback(result));
	}
	function preview() {
		const editor = vscode.window.activeTextEditor;// Get the active text editor
		if (editor) {
			preview_selection = editor.selection;
			const rsx = editor.document.getText(preview_selection);
			if (rsx.length > 0) {
				html_from_rsx(rsx, (result) => {
					if (result.length > 0) {
						if (!currentPanel) {
							const panel = vscode.window.createWebviewPanel(
								'rsxpreview',
								'Rsx Preview',
								vscode.ViewColumn.Beside,
								{
									enableScripts: true
								}
							);
							panel.webview.html = `<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>Rsx Preview</title>
</head>
<body> <div id="maindiv">` + result + ` </div> </body>
<script>
	// Handle the message inside the webview
	window.addEventListener('message', event => {
		const message = event.data; // The JSON data our extension sent
		document.getElementById("maindiv").innerHTML = message.command;
	});
</script>
</html>`;
							currentPanel = panel;
							currentPanel.onDidDispose(
								() => {
									currentPanel = undefined;
								},
								undefined,
								context.subscriptions
							);
						}
						else {
							currentPanel.webview.postMessage({ command: result });
						}
					}
				});
			} else {
				vscode.window.showWarningMessage("Please select RSX fragment before invoking this command!");
			}
		}
	}
	const handle = vscode.commands.registerCommand('extension.previewRsx', () => preview());
	vscode.workspace.onDidChangeTextDocument((event: any) => {
		function remove_window() {
			preview_selection = undefined;
			if (currentPanel) {
				currentPanel.dispose();
				currentPanel = undefined;
			}
		}
		// adjust the selection based on the edits
		if (preview_selection) {
			let start: vscode.Position = preview_selection.start;
			let end: vscode.Position = preview_selection.end;
			for (let change of event.contentChanges) {
				let range_removed = change.range;
				let lines = change.text.split('\n');
				let lines_added = lines.length;
				let last_line = lines.at(-1);
				let chars_added = last_line.length;
				let chars_diff = chars_added;
				chars_diff -= range_removed.end.character - range_removed.start.character;
				if (range_removed.end.isBefore(start)) {
					if (range_removed.start.isBefore(start)) {
						if (start.line == range_removed.end.line) {
							start = start.translate(0, chars_diff);
						}
						start = start.translate(lines_added - 1 + range_removed.start.line - range_removed.end.line, 0);
					}
					else {
						remove_window();
						return;
					}
				}
				else if (range_removed.start.isBefore(start)) {
					remove_window();
					return;
				}
				if (range_removed.end.isBefore(end)) {
					if (range_removed.start.isBefore(end)) {
						if (end.line == range_removed.end.line) {
							end = end.translate(0, chars_diff);
						}
						end = end.translate(lines_added - 1 + range_removed.start.line - range_removed.end.line, 0);
					}
					else {
						remove_window();
						return;
					}
				}
				else if (range_removed.start.isBefore(end)) {
					remove_window();
					return;
				}
			}
			preview_selection = new vscode.Selection(start, end);
			const rsx = event.document.getText(preview_selection);
			html_from_rsx(rsx, (result) => {
				if (currentPanel) {
					currentPanel.webview.postMessage({ command: result });
				}
			});
		}
	});
	context.subscriptions.push(handle);
}
