export interface ProxySettings {
  enable: boolean;
  system_proxy?: boolean;
  http_proxy?: string;
  https_proxy?: string;
}

export const defaultProxySettings: ProxySettings = {
  enable: false,
  system_proxy: true,
  http_proxy: "",
  https_proxy: "",
};

interface Settings {
  proxy: ProxySettings,
}

export const defaultSettings: Settings = {
  proxy: defaultProxySettings,
};

export default Settings;