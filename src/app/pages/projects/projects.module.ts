import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProjectsComponent } from './projects.component';
import { RouterModule } from '@angular/router';
const projectsRoot = 'projects';
const routes = [
  {path: `${projectsRoot}`, component: ProjectsComponent}
];


@NgModule({
  imports: [
    CommonModule,
    RouterModule.forChild(routes)
  ],
  declarations: [ProjectsComponent],
  exports: [RouterModule]
})
export class ProjectsModule { }
