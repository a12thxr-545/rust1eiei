import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { finalize } from 'rxjs/operators';

let requestCount = 0;

export const loadingInterceptor: HttpInterceptorFn = (req, next) => {
    const spinner = inject(NgxSpinnerService);

    requestCount++;

    if (requestCount === 1) {
        spinner.show();
    }

    return next(req).pipe(
        finalize(() => {
            requestCount--;
            if (requestCount === 0) {
                spinner.hide();
            }
        })
    );
};
