import { Injectable } from '@angular/core';
import { MySQLService} from '../mysql/my-sql.service';


@Injectable({
  providedIn: 'root'
})
export class NavigationProviderService {
  constructor(private database: MySQLService) {
  }
}
