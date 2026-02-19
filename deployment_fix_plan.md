# Deployment Error Fix

The screenshot shows an error in the Render deployment dashboard: "There's an error above. Please fix it to continue." This error is likely caused by the `DATABASE_URL` placeholder in `render.yaml` and the use of brackets in the password in `.env`.

## Changes

1.  **`/Users/arthur/serverrust/server/.env`**:
    *   Remove brackets `[]` from the `DATABASE_URL` password.
    *   Clean up duplicate `SERVER_BODY_LIMIT` entries.
2.  **`/Users/arthur/serverrust/render.yaml`**:
    *   Update `DATABASE_URL` with the actual connection string from `.env`.
    *   Add missing environment variables seen in the deployment screenshot (`JWT_USER_REFRESH_SECRET`, `MAX_CREW_PER_MISSION`, `JWT_LIFE_TIME_DAYS`).
    *   Update `SERVER_BODY_LIMIT` to a more reasonable value (2MB as in `.env` or matched to the user's intent).

3.  **Dockerfile Location**:
    *   The `Dockerfile` was located in `/server/Dockerfile`, but Render looks for it in the root by default.
    *   Created a `/Dockerfile` in the root that correctly points to the `server` source code.
    *   Updated `render.yaml` to use `runtime: docker` with the correct context and path.

## Verification

*   Check that `render.yaml` no longer contains the `[YOUR-PASSWORD]` placeholder.
*   Confirm that the root `Dockerfile` exists.
*   Ask the user to push these changes and trigger a manual deploy in Render.
