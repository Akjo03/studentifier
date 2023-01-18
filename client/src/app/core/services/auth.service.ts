import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { User } from 'src/app/shared/models/user.model';
import { environment } from '../../../environment/environment';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private minutesLeft: number = 60;
  private interval: any;
  private url = environment.api_url + "/api/v1/auth/";
  constructor(private http: HttpClient) { }
  
  public login(userData: User){
    const endpoint = this.url + "login";
    const body = JSON.stringify(userData)
    const headers = new HttpHeaders({'Content-Type': 'application/json'})
    this.http.post(endpoint, body, { headers: headers}).subscribe((data:any) => {
      localStorage.setItem('access_token', data.access_token);
      localStorage.setItem('refresh_token', data.refresh_token);
    });
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
            this.minutesLeft -= 1;
        } else {
            this.refresh();
            this.minutesLeft = 30;
        }
    },60000)
  }
}