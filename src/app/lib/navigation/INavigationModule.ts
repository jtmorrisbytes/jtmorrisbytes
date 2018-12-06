import { INavigationComponent } from './INavigationComponent';

export interface INavigationModule {
    name: string;
    path: string;
    objectReference: any;
    children: Array<INavigationModule | INavigationComponent>;
}
