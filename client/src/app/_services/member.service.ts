import { HttpClient } from "@angular/common/http";
import { inject, Injectable, signal, PLATFORM_ID } from "@angular/core";
import { isPlatformBrowser } from "@angular/common";
import { environment } from "../../environments/environment.development";
import { default_pagination, Pagination, User, UserPagination } from "../_model/pagination";
import { CacheManager } from "../_helpers/cache.helper";
import { parseQuery } from "../_helpers/query.helper";

type dataType = 'members' | 'followers' | 'following';

@Injectable({
    providedIn: 'root'
})
export class MemberService {
    private _http = inject(HttpClient);
    private apiUrl = environment.base_url + '/api/';
    private _platformId = inject(PLATFORM_ID);

    paginator = signal<Pagination<UserPagination, User>>(default_pagination);

    getMember() {
        this.getData('members');
    }

    getFollowers() {
        this.getData('followers');
    }

    getFollowing() {
        this.getData('following');
    }

    private getData(opt: dataType) {
        if (!isPlatformBrowser(this._platformId)) return;
        const pagination = this.paginator().pagination;
        let key = CacheManager.createKey(pagination);

        // Find data in cache
        const cacheData = CacheManager.load(key, opt);
        if (cacheData) {
            console.log('load member from cache');
            this.paginator.set(cacheData);
            return;
        }

        console.log('load member from api');
        let url = this.apiUrl + 'user/' + parseQuery(pagination);
        if (opt === 'members') {
            url = this.apiUrl + 'brawlers/search' + parseQuery(pagination);
        }
        this._http.get<Pagination<UserPagination, User>>(url).subscribe({
            next: resp => {
                key = CacheManager.createKey(resp.pagination);
                const paginationData = CacheManager.save(key, opt, resp);
                this.paginator.set(paginationData);
            }
        });
    }

    getProfileByUsername(username: string) {
        return this._http.get<User>(this.apiUrl + 'brawlers/' + username);
    }
}
