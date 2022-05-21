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
		console.log("preview");
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
	const counter = document.getElementById('lines-of-code-counter');

	let count = 0;
	setInterval(() => {
		counter.textContent = count++;
	}, 100);

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
		if (currentPanel) {
			// for (let change of event.contentChanges) {
			// 	preview_selection.end = min(preview_selection.end, change.range.end);
			// }
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
