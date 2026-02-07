export interface MissionRating {
    id: number;
    missionId: number;
    brawlerId: number;
    brawlerName: string;
    brawlerAvatar?: string;
    rating: number;
    comment?: string;
    createdAt: string;
}

export interface MissionRatingSummary {
    averageRating: number;
    totalRatings: number;
    ratings: MissionRating[];
}

export interface AddRatingRequest {
    rating: number;
    comment?: string;
}
