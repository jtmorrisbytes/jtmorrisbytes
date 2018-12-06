import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProjectsIndexComponent } from './index/index.component';
import { RouterModule } from '@angular/router';
const projectsRoot = 'projects';
const routes = [
  {path: '', component: ProjectsIndexComponent}
];


@NgModule({
  imports: [
    CommonModule,
    RouterModule.forChild(routes)
  ],
  declarations: [ProjectsIndexComponent],
  exports: [RouterModule]
})
export class ProjectsModule { }
