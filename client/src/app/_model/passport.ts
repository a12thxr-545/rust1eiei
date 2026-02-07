export interface Passport {
    id: number;
    token_type: string;
    access_token: string;
    expires_in: number;
    display_name: string;
    avatar_url?: string;
    cover_url?: string;
}

export interface RegisterBrawlerModel {
    username: string;
    password: string;
    display_name: string;
}

export interface LoginModel {
    username: string;
    password: string;
}
