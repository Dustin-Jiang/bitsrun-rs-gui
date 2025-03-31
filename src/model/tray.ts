import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu, MenuOptions } from "@tauri-apps/api/menu";
import { TrayIcon, TrayIconOptions } from "@tauri-apps/api/tray";
import { Window } from "@tauri-apps/api/window";
import { exit } from '@tauri-apps/plugin-process';
import I18n from "./i18n";

export default class Tray {
  private static instance: Tray;
  private trayIcon: TrayIcon | undefined
  private _ready = false
  private i18n = I18n.getInstance();
  private constructor() { this.init() }

  private async init() {
    const menu = await Menu.new(this.getMenu())

    const options: TrayIconOptions = {
      id: "tray",
      menu,
      menuOnLeftClick: false,
      icon: (await defaultWindowIcon())!,
      action: async (event) => {
        switch (event.type) {
          case 'DoubleClick':
            console.log(`mouse ${event.button} button pressed`);
            if (event.button !== "Left") return;
            await Window.getCurrent().show()
            await Window.getCurrent().setFocus()
            break;
        }
      },
    }

    this.trayIcon = await TrayIcon.new(options)
    this._ready = true
  }

  private getMenu(): MenuOptions {
    return {
      items: [
        {
          id: "show",
          text: this.i18n.t("show_window"),
          action: async () => {
            await Window.getCurrent().show()
            await Window.getCurrent().setFocus()
          }
        },
        {
          id: "quit",
          text: this.i18n.t("quit"),
          action: async () => {
            await exit(0)
          }
        }
      ]
    }
  }

  static async getInstance() {
    if (this.instance) {
      return this.instance;
    }
    this.instance = new Tray();
  }
}