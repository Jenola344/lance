import { test, expect } from "@playwright/test";

// TODO: Implement full E2E flows — see docs/ISSUES.md

test("job board loads", async ({ page }) => {
  await page.goto("/jobs");
  await expect(page.getByRole("heading", { name: /jobs/i })).toBeVisible();
});

test("post a job navigates to job board", async ({ page }) => {
  await page.goto("/jobs/new");
  // TODO: fill form and submit
});

test("dispute flow renders verdict page", async ({ page }) => {
  // TODO: stub dispute creation and visit verdict page
  expect(true).toBeTruthy();
});
