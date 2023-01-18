import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { User } from 'src/app/shared/models/user.model';
import { environment } from '../../../environment/environment';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private minutesLeft: number = 10;
  private interval: any;
  private url = environment.api_url + "/api/v1/auth/";
  constructor(private http: HttpClient, private router: Router) { }
  
  public login(userData: User){
    const endpoint = this.url + "login";
    const body = JSON.stringify(userData)
    const headers = new HttpHeaders({'Content-Type': 'application/json'})
    this.http.post(endpoint, body, { headers: headers}).subscribe((data:any) => {
      localStorage.setItem('access_token', data.access_token);
      localStorage.setItem('refresh_token', data.refresh_token);
      this.router.navigate(['/students']);
    });
    this.startTimer();
  }

  public isLoggedIn() {
    return localStorage.getItem('access_token') !== null;
  }

  public refresh() {
    const endpoint = this.url + "refresh";
    let obj = {};
    Object.assign(obj, {"refresh_token": localStorage.getItem('refresh_token')})
    const body = JSON.stringify(obj)
    const headers = new HttpHeaders({'Content-Type': 'application/json'})
    this.http.post(endpoint, body, { headers: headers}).subscribe((data:any) => {
      localStorage.setItem('access_token', data.access_token);
      localStorage.setItem('refresh_token', data.refresh_token);
    });
  }
  
  public logout(){
    const endpoint = this.url + "logout";
    let obj = {};
    Object.assign(obj, {"refresh_token": localStorage.getItem('refresh_token')})
    const body = JSON.stringify(obj);
    const headers = new HttpHeaders({'Content-Type': 'application/json'})
    this.http.post(endpoint, body, { headers: headers}).subscribe();
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
  }

  startTimer() {
    this.interval = setInterval(() => {
        if (this.minutesLeft > 0) {
            this.minutesLeft--;
        } else {
            this.refresh();
            this.minutesLeft = 10;
        }
    },60000)
  }
}