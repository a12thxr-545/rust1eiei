// Error response interface from server
export interface ApiError {
    status: number;
    message: string;
    error?: string;
    details?: string;
    timestamp?: string;
}

// Generic result type for API calls
export type ApiResult<T> = {
    success: true;
    data: T;
} | {
    success: false;
    error: ApiError;
};

// Helper function to create error
export function createApiError(status: number, message: string, details?: string): ApiError {
    return {
        status,
        message,
        details,
        timestamp: new Date().toISOString()
    };
}

// Common HTTP error codes
export const HttpErrorCodes = {
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    FORBIDDEN: 403,
    NOT_FOUND: 404,
    INTERNAL_SERVER_ERROR: 500,
    BAD_GATEWAY: 502,
    SERVICE_UNAVAILABLE: 503,
    GATEWAY_TIMEOUT: 504
} as const;
