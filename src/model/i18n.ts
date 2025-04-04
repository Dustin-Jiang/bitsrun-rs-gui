import en from "./i18n/en";
import zh_CN from "./i18n/zh_CN";

const msg = {
  zh_CN: zh_CN,
  en: en
}

type Locale = keyof typeof msg;
type Msg = keyof typeof msg[Locale];

class I18n {
  private _locale: Locale = "zh_CN";
  private static instance: I18n;
  private constructor() {
    const lang = navigator.language
    const map = new Map<string, Locale>([
      ["zh_CN", "zh_CN"],
      ["en", "en"],
      
      ["zh", "zh_CN"],
      ["zh-CN", "zh_CN"],
      ["zh-Hans", "zh_CN"],
      ["zh-Hant", "zh_CN"],
      ["en-US", "en"],
      ["en-GB", "en"],
      ["en-CA", "en"],
      ["en-AU", "en"],
      ["en-NZ", "en"],
    ])
    if (map.has(lang)) {
      this._locale = map.get(lang)!
    }
  }

  static getInstance(): I18n {
    if (!I18n.instance) {
      I18n.instance = new I18n();
    }
    return I18n.instance;
  }
  
  get locale(): Locale {
    return this._locale;
  }

  set locale(locale: Locale) {
    this._locale = locale;
    document.documentElement.setAttribute("lang", locale);
  }

  t(key: Msg): string {
    return msg[this._locale][key];
  }

  format(template: Msg, ...values: string[]): string {
    return msg[this._locale][template].replace(/%s/g, () => values.shift() || '');
  }
}

export default I18n;
export type { Locale, Msg };
