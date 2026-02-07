export interface ChatMessage {
    id: number;
    missionId: number;
    brawlerId: number;
    brawlerName: string;
    brawlerAvatar?: string;
    content: string;
    createdAt: string;
    imageUrl?: string;
}

export interface SendMessage {
    content: string;
    imageUrl?: string;
}

