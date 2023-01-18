import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from '../../../environment/environment';
import { Student } from 'src/app/shared/models/student.model';
@Injectable({
  providedIn: 'root'
})
export class StudentService {
  private url = environment.api_url + "/api/v1/persons/all";
  constructor(private http: HttpClient) { }
  
  public get() {
    const headers = new HttpHeaders(
      {
        'Content-Type': 'application/json', 
        'Authorization': `Bearer ${localStorage.getItem("access_token")}`
      })
    return this.http.get(this.url, { headers: headers});
  }
}