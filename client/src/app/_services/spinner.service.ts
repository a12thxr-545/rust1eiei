import { Injectable, signal } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class NgxSpinnerService {
    private _spinners = new Map<string, ReturnType<typeof signal<boolean>>>();

    private _get(name: string = 'primary') {
        if (!this._spinners.has(name)) {
            this._spinners.set(name, signal(false));
        }
        return this._spinners.get(name)!;
    }

    show(name: string = 'primary', _config?: unknown) {
        this._get(name).set(true);
    }

    hide(name: string = 'primary') {
        this._get(name).set(false);
    }

    isVisible(name: string = 'primary') {
        return this._get(name);
    }
}
