import { NgModule, Injectable       } from '@angular/core';
import { CommonModule   } from '@angular/common';
import { HomeComponent  } from './home/home.component';
import { AboutComponent } from './about/about.component';
import { ProjectsModule } from './projects/projects.module';
import { E404Component  } from './e404/e404.component';
import { E500Component  } from './e500/e500.component';
import { RouterModule } from '@angular/router';
import { Route } from '@angular/compiler/src/core';
import { NavigationProviderService } from '@app/services/navigation/navigation-provider.service';
import { type } from 'os';


// const NavigationProvider = new NavigationProviderService()
const appNavigationItems = []

const defaultRoutes = [
  {path: ''  , component: HomeComponent  },
  {path: '**', component: E404Component }
];
const appRoutes = [].concat(appNavigationItems, defaultRoutes)
@Injectable({
  providedIn: 'root'
})


@NgModule({
  imports: [
    RouterModule.forRoot(appRoutes),
    ProjectsModule,
  ],
  declarations: [
    HomeComponent,
    E404Component,
    E500Component,
  ]
  ,
  exports: [RouterModule]
})
export class AppPagesModule {
  static rootDir: string = "";
  children: [];
  constructor() {
  }
}
