export interface Passport {
    id: number;
    token_type: string;
    access_token: string;
    expires_in: number;
    display_name: string;
    username: string;
    avatar_url?: string;
    cover_url?: string;
    bio?: string;
    joined_count?: number;
    completed_count?: number;
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
