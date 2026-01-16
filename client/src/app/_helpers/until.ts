import { Brawler } from "../_model/brawler";

const _default_avatar = '/assets/defaultavtar.jpg';

export function getAvatarUrl(url: Brawler): string {
    if (url.avatar_url) return url.avatar_url;
    return _default_avatar;
}