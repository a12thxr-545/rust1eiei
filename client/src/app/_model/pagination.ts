export interface UserPagination {
    currentPage: number;
    pageSize: number;
    length?: number;
    query?: string;
}

export interface Pagination<T, U> {
    pagination: T;
    items: U[];
}

export const default_pagination: Pagination<UserPagination, User> = {
    pagination: {
        currentPage: 1,
        pageSize: 10,
        length: 0
    },
    items: []
}

export interface User {
    id: number;
    username: string;
    display_name: string;
    avatar_url?: string;
    cover_url?: string;
    photoOfTheDay?: string;
    bio?: string;
    age?: number;
    joined_count?: number;
    completed_count?: number;
}
