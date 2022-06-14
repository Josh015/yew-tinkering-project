mod main_content;
mod toolbar;

use main_content::*;
use material_yew::{
    drawer::{MatDrawerAppContent, MatDrawerTitle},
    top_app_bar_fixed::{
        MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle,
    },
    MatButton, MatCircularProgress, MatDrawer, MatIconButton, MatList,
    MatListItem, MatTopAppBarFixed,
};
use std::num::NonZeroU64;
use toolbar::*;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::AppRoute;

const SMALL_WIDTH_BREAKPOINT: f64 = 720.0;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub user_id: Option<NonZeroU64>,
}

/// Checks whether we're on a small screen or not.
///
/// Must be called from a function component.
pub fn is_small_window() -> bool {
    let (width, _) = use_window_size();
    width < SMALL_WIDTH_BREAKPOINT
}

#[function_component(ContactManager)]
pub fn contact_manager(props: &Props) -> Html {
    let is_small_window = is_small_window();
    let is_drawer_open = use_bool_toggle(!is_small_window);

    // Ensure that the drawer is open for large windows.
    if !is_small_window && !*is_drawer_open {
        is_drawer_open.set(true);
    }

    let onopened = {
        let is_drawer_open = is_drawer_open.clone();
        Callback::from(move |_| is_drawer_open.set(true))
    };

    let onclosed = {
        let is_drawer_open = is_drawer_open.clone();
        Callback::from(move |_| is_drawer_open.set(false))
    };

    let onnavigationiconclick = {
        let is_drawer_open = is_drawer_open.clone();
        Callback::from(move |_| is_drawer_open.toggle())
    };

    // TODO: How to fetch users list exactly once?
    // TODO: Need a context for holding list of users that child components can add to.
    // TODO: Retrieve user info via ID.
    // TODO: Figure out how to not have vertical menu button hidden when drawer is open for large screen mode!

    let is_loading = false;
    let ids = [1u64, 2, 3, 4];

    html! {
        <MatDrawer
            drawer_type={if is_small_window { "modal" } else { "dismissible" }}
            open={*is_drawer_open}
            {onopened}
            {onclosed}
        >
            // <mat-sidenav-container
            //   class="app-sidenav-container"
            //   [class.dark-theme]="isDarkTheme$ | async"
            //   [dir]="(textDirection$ | async) || 'ltr'"
            // >
            //   <mat-sidenav
            //     class="app-sidenav mat-elevation-z10"
            //   >

            <MatDrawerTitle>
                <span class="drawer-title">{ "TODO" }</span>
            </MatDrawerTitle>
            //     <mat-toolbar color="primary">
            //       {{ 'ContactManager.Sidenav.Title' | translate }}
            //     </mat-toolbar>

            <div class="drawer-content">
                {if is_loading {
                    //     <ng-container *ngIf="usersLoading$ | async; else usersLoaded">
                    //       <mat-spinner style="margin: 0 auto"></mat-spinner>
                    //     </ng-container>
                    html! {
                        <MatCircularProgress indeterminate=true />
                    }
                } else {
                    //       <mat-nav-list>
                    //         <mat-list-item *ngFor="let user of users$ | async">
                    //           <a
                    //             matLine
                    //             routerLinkActive="active-list-item"
                    //             [routerLink]="['/contact-manager', user.id]"
                    //           >
                    //             <mat-icon [svgIcon]="user.avatar"></mat-icon> {{ user.name }}
                    //           </a>
                    //         </mat-list-item>
                    //       </mat-nav-list>
                    html! {
                        <MatList>
                            {for ids.map(|id| {
                                html! {
                                    <Link<AppRoute> to={AppRoute::ContactManagerView { user_id: NonZeroU64::new(id).unwrap() }}>
                                        <MatListItem>{ id }</MatListItem>
                                    </Link<AppRoute>>
                                }
                            })}
                        </MatList>
                    }
                }}
            </div>

            <MatDrawerAppContent>
                <div class="app-content">
                    <Toolbar {onnavigationiconclick} />
                    //     <app-toolbar
                    //       (toggleTheme)="toggleTheme()"
                    //       (toggleSidenav)="sidenav.toggle()"
                    //       (toggleDir)="toggleDir()"
                    //     ></app-toolbar>

                    <main id="router-outlet">
                        <MainContent id={props.user_id} />
                    </main>
                </div>
            </MatDrawerAppContent>
        </MatDrawer>
    }
}

// .app-sidenav-container {
//     flex: 1;
//     width: 100%;
//     min-width: 100%;
//     height: 100%;
//     min-height: 100%;
//   }

//   .app-sidenav-content {
//     display: flex;
//     height: 100%;

//     flex-direction: column;
//   }

//   .app-sidenav {
//     width: 240px;
//   }

//   .wrapper {
//     margin: 50px;
//   }

//   .active-list-item {
//     color: #3f51b5 !important; /* Note: You could also use a custom theme */
//   }

//   import { BreakpointObserver, BreakpointState } from '@angular/cdk/layout';
//   import {
//     ChangeDetectionStrategy,
//     Component,
//     OnInit,
//     ViewChild,
//   } from '@angular/core';
//   import { MatSidenav } from '@angular/material/sidenav';
//   import { Router } from '@angular/router';
//   import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
//   import { Store } from '@ngrx/store';

//   import { getUsers, getUsersLoading, State } from '../../store';
//   import {
//     getIsDarkTheme,
//     getTextDirection,
//     toggleDarkTheme,
//     toggleTextDirection,
//   } from 'src/app/store';

//   @UntilDestroy()
//   @Component({
//     selector: 'app-sidenav',
//     templateUrl: './sidenav.component.html',
//     styleUrls: ['./sidenav.component.scss'],
//     changeDetection: ChangeDetectionStrategy.OnPush,
//   })
//   export class SidenavComponent implements OnInit {
//     static readonly smallWidthBreakpoint = 720;

//     readonly usersLoading$ = this.store.select(getUsersLoading);
//     readonly users$ = this.store.select(getUsers);
//     readonly isDarkTheme$ = this.store.select(getIsDarkTheme);
//     readonly textDirection$ = this.store.select(getTextDirection);

//     isScreenSmall = false;

//     @ViewChild(MatSidenav) sidenav?: MatSidenav;

//     constructor(
//       private readonly breakpointObserver: BreakpointObserver,
//       private readonly router: Router,
//       private readonly store: Store<State>
//     ) {}

//     ngOnInit(): void {
//       this.breakpointObserver
//         .observe([`(max-width: ${SidenavComponent.smallWidthBreakpoint}px)`])
//         .pipe(untilDestroyed(this))
//         .subscribe((state: BreakpointState) => {
//           this.isScreenSmall = state.matches;
//         });

//       this.router.events.pipe(untilDestroyed(this)).subscribe(() => {
//         if (this.isScreenSmall) {
//           this.sidenav?.close();
//         }
//       });
//     }

//     toggleTheme(): void {
//       this.store.dispatch(toggleDarkTheme());
//     }

//     toggleDir(): void {
//       this.store.dispatch(toggleTextDirection());
//     }
//   }
