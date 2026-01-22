import { Injectable, inject } from '@angular/core';
import { MatSnackBar, MatSnackBarConfig } from '@angular/material/snack-bar';

export type SnackbarType = 'success' | 'error' | 'warning' | 'info';

@Injectable({
    providedIn: 'root',
})
export class SnackbarService {
    private snackBar = inject(MatSnackBar);

    private readonly defaultConfig: MatSnackBarConfig = {
        duration: 4000,
        horizontalPosition: 'end',
        verticalPosition: 'top',
    };

    private getConfigByType(type: SnackbarType): MatSnackBarConfig {
        return {
            ...this.defaultConfig,
            panelClass: [`snackbar-${type}`],
        };
    }

    show(message: string, type: SnackbarType = 'info', action: string = 'Close') {
        this.snackBar.open(message, action, this.getConfigByType(type));
    }

    success(message: string, action: string = 'OK') {
        this.show(message, 'success', action);
    }

    error(message: string, action: string = 'Close') {
        this.show(message, 'error', action);
    }

    warning(message: string, action: string = 'Close') {
        this.show(message, 'warning', action);
    }

    info(message: string, action: string = 'Close') {
        this.show(message, 'info', action);
    }
}
