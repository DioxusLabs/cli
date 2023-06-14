# Plugin

`dioxus plugin` can help developer manage & develop their own cli plugin.

### Init plugin system

This command will download a plugin core library in your project, and enable plugin system.

```shell
dioxus plugin init
```



### Install & add new plugin

This command can help you install a new plugin from git.

> currently it just support install from git repository.

```shell
dioxus plugin add --git {GIT_URL}
```



### Upgrade core library

Beacause of the core library must be sync with cli version, so you need use upgrade core library after updated cli version.

```shell
dioxus plugin upgrade
```
