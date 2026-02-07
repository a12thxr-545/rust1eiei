import { Pagination, User, UserPagination } from "../_model/pagination";
import { PhotoHelper } from "./photo.helper";

type cacheValue = Pagination<UserPagination, User>;
const CACHE_TTL = 5 * 60 * 1000; // 5 minutes

interface CachedItem {
    value: cacheValue;
    timestamp: number;
}

const data = new Map<string, CachedItem>();

type cacheOpt = 'members' | 'followers' | 'following' | 'chat';

export const CacheManager = {
    clear: function (opt: string = '') {
        console.log('clear cache');
        if (opt !== '') {
            for (const key of data.keys()) {
                if (key.startsWith(opt))
                    data.delete(key);
            }
        } else {
            data.clear();
        }
    },

    save: function (key: string, opt: cacheOpt, value: cacheValue): cacheValue {
        if (opt !== 'chat') {
            value.items = value.items.map(u => PhotoHelper.parseUser(u as User));
        }
        data.set(opt + key, { value, timestamp: Date.now() });
        return value;
    },

    load: function (key: string, opt: cacheOpt): cacheValue | undefined {
        const item = data.get(opt + key);
        if (item) {
            const now = Date.now();
            if (now - item.timestamp > CACHE_TTL) {
                data.delete(opt + key);
                return undefined;
            }
            return item.value;
        }
        return undefined;
    },

    createKey: function <T extends { [key: string]: any }>(query: T): string {
        return Object.values(query).join('_');
    }
};
