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
  
  public login(userData: User) {
    const endpoint = this.url + "login";
    let obj = {};
    Object.assign(obj, {"username": userData.username, "password": userData.password, "role": userData.username.toLowerCase()})
    const body = JSON.stringify(obj)
    const headers = new HttpHeaders({'Content-Type': 'application/json'})
    return this.http.post(endpoint, body, { headers: headers});
  }

  public setToken(access: string, refresh: string) {
    localStorage.setItem('access_token', access);
    localStorage.setItem('refresh_token', refresh);
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

  public startTimer() {
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