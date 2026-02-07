import { ApplicationConfig, importProvidersFrom, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter, withInMemoryScrolling, withRouterConfig } from '@angular/router';
import { provideHttpClient, withFetch, withInterceptors } from '@angular/common/http';
import { loadingInterceptor } from './_interceptor/loading-interceptor';
import { jwtInterceptor } from './_interceptor/jwt-interceptor';
import { errorInterceptor } from './_interceptor/error-interceptor';
import { routes } from './app.routes';
import { provideClientHydration, withEventReplay } from '@angular/platform-browser';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { NgxSpinnerModule } from 'ngx-spinner';
import { MatSnackBarModule } from '@angular/material/snack-bar';

export const appConfig: ApplicationConfig = {
  providers: [
    importProvidersFrom(NgxSpinnerModule, MatSnackBarModule),
    provideBrowserGlobalErrorListeners(),
    provideAnimationsAsync(),
    provideRouter(routes,
      withInMemoryScrolling({ scrollPositionRestoration: 'enabled' }),
      withRouterConfig({ onSameUrlNavigation: 'reload' })
    ),
    provideClientHydration(withEventReplay()),
    provideHttpClient(withFetch(), withInterceptors([jwtInterceptor, errorInterceptor, loadingInterceptor])),
  ]
};
