import { Injectable } from '@angular/core';
import { HomeComponent } from '@app/pages/home/home.component';
import { AboutComponent } from '@app/pages/about/about.component';
import { ProjectsIndexComponent } from '@app/pages/projects/index/index.component';
import { Component, NgModule } from '@angular/compiler/src/core';

import { INavigationComponent } from '@app/lib/navigation/INavigationComponent';
import { ProjectsModule } from '@app/pages/projects/projects.module';
import { SacrificalGoatModule } from '@app/goats/sacrifical-goat/sacrifical-goat.module';
import { INavigationTree } from '@app/lib/navigation/INavigationTree';
import { INavigationModule } from '@app/lib/navigation/INavigationModule';
import { AppComponent } from '@app/app.component';
import { BlackBabyGoatComponent } from '@app/goats/sacrifical-goat/black-baby-goat/black-baby-goat.component';
import { WhiteBabyGoatComponent } from '@app/goats/sacrifical-goat/white-baby-goat/white-baby-goat.component';
import { SacrificialLambModule } from '@app/goats/sacrifical-goat/sacrificial-lamb/sacrificial-lamb.module';
import { RedLambComponent } from '@app/goats/sacrifical-goat/sacrificial-lamb/red-lamb/red-lamb.component';
import { GreenLambComponent } from '@app/goats/sacrifical-goat/sacrificial-lamb/green-lamb/green-lamb.component';
import { DirectoryProvider } from '../directory/directory-provider.service';

const appPagesRoot = '@app/pages/';
const appPagesModuleLocation = appPagesRoot + 'app-pages.module';

import { AppPagesModule } from '@app/pages/app-pages.module';

@Injectable({
  providedIn: 'root'
})
export class NavigationProviderService {
  rootModule = new AppPagesModule();
  ItemsToEnumerate;
  navigationTree: INavigationModule;
  constructor(directoryProvider: DirectoryProvider) {
    directoryProvider.rootDirectory = appPagesRoot;
    this.ItemsToEnumerate = {
      root: this.rootModule,
      children:[ 
        { objectReference: AboutComponent}
      ]
    }
    //this.generateNavLinks();
    this.navigationTree = {
      name: "App Navigation root",
      path: "",
      objectReference: this.rootModule,
      children:[
        
      ]
    };
    this.generateNavigationTree();
    
  }
  generateNavigationTree(){
  // console.log("generating navigation tree");
  // console.log(" root object on following line");
  let rootModule = this.navigationTree.objectReference;
  //console.log( rootModule )

  }
  searchEnumerationTreeForChildren(children:any, result:[{}] = [{}]){
    if(children){
      // console.log("children was defined")
      // console.log(children);
      if(children.length > 0){
        if(children.objectReference){
          
        }
        console.log("recursive search found children");
        console.log(children[0])
        
      }
      
    }
    else{
      console.log("children was undefined or null")
      console.log(children)
    }
    
  }
  generateNavigationComponent(objectReference:any){
      
  }


  // generateNavLinks(){
  //   for(let i=0; i < this.components.length; i++){
  //     let currentComponent = this.components[i];
  //     let currentComponentInstance = new currentComponent;
  //     let navigationItem:INavigationItem = 
  //     {
  //       path: currentComponentInstance.path,
  //       component: currentComponent,
  //       children: [],
  //       title: currentComponentInstance.title,
  //       titlebarText: currentComponentInstance.titlebarText
  //     }
  //     this.navigationItems.push(navigationItem);
    
        
  //   } 
    
}

