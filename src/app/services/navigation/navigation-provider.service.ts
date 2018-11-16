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

