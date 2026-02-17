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
            // Don't log 401/400 to console, and skip global 401 handling for login endpoint
            if (error.status === 401 && !req.url.includes('/authentication/login')) {
                passportService.removePassport();
                router.navigate(['/login']);
            }
            return throwError(() => error);
        })
    );
};
