import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AppService } from './core/services/app.service';
import { AuthService } from './core/services/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'Studentifier Client';
  showMenu = true;

  constructor(public appService: AppService, private authService: AuthService, private router: Router) {}

  toggleNavbar() {
    this.showMenu = !this.showMenu;
  }

  logout() {
    this.appService.set_isLoggedIn_false();
    this.authService.logout();
    this.router.navigate(["/"]);
  }
}
