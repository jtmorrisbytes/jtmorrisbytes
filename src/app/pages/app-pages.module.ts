import { NgModule       } from '@angular/core';
import { CommonModule   } from '@angular/common';
import { HomeComponent  } from './home/home.component';
import { AboutComponent } from './about/about.component';
import { ProjectsModule } from './projects/projects.module';
import { E404Component  } from './e404/e404.component';
import { E500Component  } from './e500/e500.component';
import { RouterModule } from '@angular/router';
import { Route } from '@angular/compiler/src/core';

const routes = [
  {path: ''  , component: HomeComponent },
  {path: 'about', component: AboutComponent},
  {path: '**', component: E404Component }
];


@NgModule({
  imports: [
    RouterModule.forRoot(routes),
    CommonModule,
    ProjectsModule,
  ],
  declarations: [
    HomeComponent,
    AboutComponent,
    E404Component,
    E500Component,
  ],
  exports: [RouterModule]
})
export class AppPagesModule {
  constructor() {
    console.log("hello from appPagesModule");
    console.log(RouterModule)
    
  }
}
