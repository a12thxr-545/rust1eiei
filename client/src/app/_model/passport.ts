export interface Passport {
    token: string;
    display_name: string;
    avatar_url?: string;
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


