import { Component, OnInit } from '@angular/core';
import { FormBuilder } from '@angular/forms';
import { Student } from 'src/app/shared/models/student.model';
import { AppService } from '../../services/app.service';

@Component({
  selector: 'app-student-list',
  templateUrl: './student-list.component.html',
  styleUrls: ['./student-list.component.css']
})
export class StudentListComponent implements OnInit {
  
  searchForm: any;
  searchTerm: string = '';
  filterList: Student[] = [];
  students: Student[] = [
    {picture: "../../../../assets/graduated.png", firstname:"Aline", lastname:"Ammann", street:"Strasse 1", city: "Kandersteg", email: "aline@gmail.com"},
    {picture: "../../../../assets/graduated.png", firstname:"Lukas", lastname:"Küffer", street:"Strasse 2", city: "Thörishus", email: "lukas@gmail.com"},
    {picture: "../../../../assets/graduated.png", firstname:"Svenja", lastname:"Berger", street:"Strasse 3", city: "Thun", email: "svenja@gmail.com"},
    {picture: "../../../../assets/graduated.png", firstname:"Anna", lastname:"Müller", street:"Strasse 4", city: "Bern", email: "anna@gmail.com"},
    {picture: "../../../../assets/graduated.png", firstname:"Adshahran", lastname:"Kirubakaran", street:"Strasse 5", city: "Zürich", email: "nico@gmail.com"},
    {picture: "../../../../assets/graduated.png", firstname:"Manuel", lastname:"Hofer", street:"Strasse 6", city: "Luzern", email: "manuel@gmail.com"},
    {picture: "../../../../assets/graduated.png", firstname:"Nina", lastname:"Ammann", street:"Strasse 1", city: "Kandersteg", email: "aline@gmail.com"},
  ];
  activeList = this.students;
  filter_active = false;

  constructor(private appService: AppService, private formBuilder: FormBuilder) {}

  ngOnInit(): void {
    this.appService.set_isLoggedIn_true();
    this.searchForm = this.formBuilder.group({
      searchTerm: ['']
    })
  }
  
  filter() {
    this.searchTerm = this.searchForm.get('searchTerm').value;
    this.filterList = [];
    this.filter_active = true;
    this.students.forEach(student => {
      if(student.firstname == this.searchTerm || student.lastname == this.searchTerm) {
        this.filterList.push(student);
      }
    });
    this.activeList = this.filterList;
  }

  clearFilter() {
    this.searchTerm = '';
    this.filter_active = false;
    this.activeList = this.students;
  }
}
