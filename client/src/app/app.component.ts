import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from './core/services/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'Studentifier Client';
  showMenu = true;

  constructor(public authService: AuthService, private router: Router) {}

  toggleNavbar() {
    this.showMenu = !this.showMenu;
  }

  logout() {
    this.authService.logout();
    this.router.navigate(["/"]);
  }
}
