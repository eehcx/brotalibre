use std::path::Path;

use anyhow::{bail, Result};

use crate::infrastructure::seeder::commands::write_file;

pub(crate) fn apply_cdp_architecture_template(project_dir: &Path) -> Result<()> {
    let app_dir = project_dir.join("src/app");
    if !app_dir.exists() {
        bail!(
            "could not find Angular app directory at `{}`",
            app_dir.display()
        );
    }

    write_file(
        &app_dir.join("core/models/health-status.model.ts"),
        r#"export interface HealthStatus {
  service: string;
  status: 'ok' | 'degraded';
  checkedAt: string;
}
"#,
    )?;

    write_file(
        &app_dir.join("core/environment/app-environment.ts"),
        r#"export const appEnvironment = {
  appName: 'ngseed-cdp-app',
  apiBaseUrl: '/api',
};
"#,
    )?;

    write_file(
        &app_dir.join("core/commons/logger.ts"),
        r#"export function logInfo(message: string): void {
  console.info(`[CDP] ${message}`);
}
"#,
    )?;

    write_file(
        &app_dir.join("core/auth/auth.types.ts"),
        r#"export interface AuthUser {
  id: string;
  role: string;
}
"#,
    )?;

    write_file(
        &app_dir.join("data/datasource/remote/health.datasource.ts"),
        r#"import { Injectable } from '@angular/core';
import { HealthStatus } from '../../../core/models/health-status.model';

@Injectable({ providedIn: 'root' })
export class HealthRemoteDataSource {
  getStatus(): HealthStatus {
    return {
      service: 'ngseed-cdp',
      status: 'ok',
      checkedAt: new Date().toISOString(),
    };
  }
}
"#,
    )?;

    write_file(
        &app_dir.join("data/datasource/local/preferences.datasource.ts"),
        r#"import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class PreferencesLocalDataSource {
  private readonly key = 'ngseed:theme';

  getTheme(): string {
    return localStorage.getItem(this.key) ?? 'light';
  }
}
"#,
    )?;

    write_file(
        &app_dir.join("presentation/pages/health/health.page.ts"),
        r#"import { CommonModule } from '@angular/common';
import { Component, inject } from '@angular/core';
import { HealthRemoteDataSource } from '../../../data/datasource/remote/health.datasource';
import { PreferencesLocalDataSource } from '../../../data/datasource/local/preferences.datasource';

@Component({
  selector: 'app-health-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './health.page.html',
})
export class HealthPage {
  private readonly remote = inject(HealthRemoteDataSource);
  private readonly local = inject(PreferencesLocalDataSource);

  readonly health = this.remote.getStatus();
  readonly theme = this.local.getTheme();
}
"#,
    )?;

    write_file(
        &app_dir.join("presentation/pages/health/health.page.html"),
        r#"<main class="shell">
  <h1>CDP Architecture Ready</h1>
  <p>Status: {{ health.status }} ({{ health.service }})</p>
  <p>Theme preference: {{ theme }}</p>
</main>
"#,
    )?;

    write_file(
        &app_dir.join("app.routes.ts"),
        r#"import { Routes } from '@angular/router';
import { HealthPage } from './presentation/pages/health/health.page';

export const routes: Routes = [
  {
    path: '',
    component: HealthPage,
  },
];
"#,
    )?;

    patch_app_component_for_cdp(&app_dir)?;
    patch_app_config_for_cdp(&app_dir)?;

    Ok(())
}

pub(crate) fn patch_app_component_for_cdp(app_dir: &Path) -> Result<()> {
    let (app_ts, app_html, template_url, style_property, component_class) =
        if app_dir.join("app.ts").exists() {
            (
                app_dir.join("app.ts"),
                app_dir.join("app.html"),
                "./app.html",
                "styleUrl",
                "App",
            )
        } else {
            (
                app_dir.join("app.component.ts"),
                app_dir.join("app.component.html"),
                "./app.component.html",
                "styleUrls",
                "AppComponent",
            )
        };

    write_file(&app_ts, &{
        let template = r#"import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: '__TEMPLATE_URL__',
  __STYLE_PROPERTY__: ['./app.scss'],
})
export class __COMPONENT_CLASS__ {}
"#;
        template
            .replace("__TEMPLATE_URL__", template_url)
            .replace("__STYLE_PROPERTY__", style_property)
            .replace("__COMPONENT_CLASS__", component_class)
    })?;

    write_file(
        &app_html,
        r#"<router-outlet />
"#,
    )?;

    Ok(())
}

pub(crate) fn patch_app_config_for_cdp(app_dir: &Path) -> Result<()> {
    let app_config = app_dir.join("app.config.ts");

    write_file(
        &app_config,
        r#"import { ApplicationConfig } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes)],
};
"#,
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn cdp_template_creates_layered_files() {
        let tmp = tempdir().unwrap();
        let app_dir = tmp.path().join("demo/src/app");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("app.ts"), "").unwrap();
        fs::write(app_dir.join("app.html"), "").unwrap();
        fs::write(app_dir.join("app.config.ts"), "").unwrap();
        fs::write(app_dir.join("app.routes.ts"), "").unwrap();

        apply_cdp_architecture_template(&tmp.path().join("demo")).unwrap();

        assert!(app_dir.join("core/models/health-status.model.ts").exists());
        assert!(app_dir
            .join("data/datasource/remote/health.datasource.ts")
            .exists());
        assert!(app_dir
            .join("presentation/pages/health/health.page.ts")
            .exists());
    }
}
