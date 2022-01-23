export interface Config {
    readonly host: string;
}

export const config: Config = {
    host: import.meta.env.VITE_SERVER_HOST,
}
