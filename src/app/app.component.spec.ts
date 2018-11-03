import { TestBed, async, ComponentFixture } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';
import { AppComponent } from './app.component';
import { AppNavComponent } from './components/nav-main/nav-main.component';
import { By } from '@angular/platform-browser';
import { FooterComponent } from './components/footer/footer.component';
import { DebugElement, DebugNode } from '@angular/core';

describe('AppComponent', () => {
  let app : AppComponent;
  let fixture : ComponentFixture<AppComponent>;
  let AppNavigation: DebugElement;
  let appElement : HTMLElement;
  beforeEach(async(() => {
    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        AppComponent,
        AppNavComponent,
        FooterComponent
      ],
    }).compileComponents();
  fixture = TestBed.createComponent(AppComponent);
  app = fixture.componentInstance;
  AppNavigation = fixture.debugElement.query(By.directive(AppNavComponent));
  appElement= fixture.nativeElement;
  }));
  

  it('should create the app', () => {
    expect(app).toBeTruthy();
  });

  it("should have a title", () => {
    expect(app.title).toBeDefined();
    expect(app.title).toBe("jtmorrisbytes.com");
  });
  it("should have a subtilte", () =>{
    expect(app.subtitle).toBeDefined("app subtitle should be defined");
    expect(app.subtitle).toBe("An experiment by Jordan Morris");
  });
  it("should create the navigation component", () => {
    
    //const AppNavigation = fixture.debugElement.query(By.directive(AppNavComponent));
    expect(AppNavigation)
    .toBeTruthy("make sure the AppNavComponent is correctly defined "+
                "in app.component.ts and app.component.html"
     );
    //const navigation = TestBed.createComponent(AppNavComponent)
    //navigation.detectChanges()
    
  });
  it("should render the app router", () => {

      
        

      //expect(fixture.nativeElement).toContain(Ap)
  });
  it("should correctly render the navigation menu", ()=>{
    
    const navElement : NodeList = appElement.querySelectorAll(AppNavigation.name)
    expect(navElement).toBeDefined();
    expect(navElement).toBeTruthy();
    expect(navElement.length)
      .toBeGreaterThan(0, 
        `There should be at least one ${AppNavigation.name} element in AppCompnent`
      );
    expect(navElement.length)
      .toBeLessThan(2,
        `There should be no more than one ${AppNavigation.name} element in AppComponent`
      );
  });
  it("should create the footer component", () => {
    const appFooter = fixture.debugElement.query(By.directive(FooterComponent));
    //console.log(fixture.debugElement.children)
    expect(appFooter).toBeTruthy("make sure the app footer is defined in " +
                                 "the AppComponent Template");
    //expect(appFooter).toBeTruthy();


  })
});
