import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class AppService {
  public isLoggedIn: boolean = false;

  public set_isLoggedIn_true() {
    this.isLoggedIn = true;
  }

  public set_isLoggedIn_false() {
    this.isLoggedIn = false;
  }
}