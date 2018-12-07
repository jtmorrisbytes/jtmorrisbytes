import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class TestService {
  greeting: string;
  constructor() {
    this.greeting = 'Hello from testService';
  }
  getGreeting() {
    return this.greeting;
  }
}
