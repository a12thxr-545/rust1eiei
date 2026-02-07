import { HttpErrorResponse, HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { catchError, throwError } from 'rxjs';
import { PassportService } from '../_services/passport-service';
import { Router } from '@angular/router';

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
    const passportService = inject(PassportService);
    const router = inject(Router);

    return next(req).pipe(
        catchError((error: HttpErrorResponse) => {
            if (error.status === 401) {
                // Unauthorized - token might be expired or invalid
                console.warn('Unauthorized request detected. Logging out user...');
                passportService.removePassport();
                router.navigate(['/login']);
            }
            return throwError(() => error);
        })
    );
};
