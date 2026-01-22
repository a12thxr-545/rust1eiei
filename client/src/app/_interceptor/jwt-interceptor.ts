import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { PassportService } from '../_services/passport-service';

export const jwtInterceptor: HttpInterceptorFn = (req, next) => {
  const passportService = inject(PassportService);
  const passport = passportService.data();

  // If user is logged in, add Authorization header
  if (passport?.access_token) {
    const clonedReq = req.clone({
      setHeaders: {
        Authorization: `Bearer ${passport.access_token}`
      }
    });
    return next(clonedReq);
  }

  return next(req);
};
