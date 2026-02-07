import { Injectable, signal, PLATFORM_ID, inject } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

export type ThemeType = 'light' | 'dark' | 'orange';

@Injectable({
    providedIn: 'root'
})
export class ThemeService {
    private platformId = inject(PLATFORM_ID);
    theme = signal<ThemeType>('dark');

    constructor() {
        if (isPlatformBrowser(this.platformId)) {
            const savedTheme = localStorage.getItem('theme') as ThemeType;
            if (savedTheme && ['light', 'dark', 'orange'].includes(savedTheme)) {
                this.setTheme(savedTheme);
            } else {
                this.setTheme('dark');
            }
        }
    }

    toggleTheme() {
        const themes: ThemeType[] = ['light', 'dark', 'orange'];
        const currentIndex = themes.indexOf(this.theme());
        const nextIndex = (currentIndex + 1) % themes.length;
        this.setTheme(themes[nextIndex]);
    }

    private setTheme(theme: ThemeType) {
        this.theme.set(theme);
        if (isPlatformBrowser(this.platformId)) {
            localStorage.setItem('theme', theme);
            document.documentElement.setAttribute('data-theme', theme);
        }
    }
}
