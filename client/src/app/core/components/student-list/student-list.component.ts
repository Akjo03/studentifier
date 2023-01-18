import { Component, OnInit } from '@angular/core';
import { FormBuilder } from '@angular/forms';
import { Student } from 'src/app/shared/models/student.model';
import { AuthService } from '../../services/auth.service';
import { StudentService } from '../../services/student.service';

@Component({
  selector: 'app-student-list',
  templateUrl: './student-list.component.html',
  styleUrls: ['./student-list.component.css']
})
export class StudentListComponent implements OnInit {
  
  searchForm: any;
  searchTerm: string = '';
  filterList: Student[] = [];
  students: Student[] = [];
  activeList = this.students;
  filter_active = false;

  constructor(private formBuilder: FormBuilder, private studentService: StudentService) {}

  ngOnInit(): void {
    setTimeout(() => {
      this.studentService.get().subscribe((data:any) => {
        setTimeout(() => {
          this.students = data.persons;
          this.clearFilter();
        }, 1)
      })
    }, 1);
    this.searchForm = this.formBuilder.group({
      searchTerm: ['']
    });
  }
  
  filter() {
    this.searchTerm = this.searchForm.get('searchTerm').value.toLocaleLowerCase();
    console.log(this.searchTerm);
    this.filterList = [];
    this.filter_active = true;
    this.filterList = this.students.filter((student) => 
        student.last_name.toLocaleLowerCase().indexOf(this.searchTerm) !== -1 ||
        student.first_name.toLocaleLowerCase().indexOf(this.searchTerm)  !== -1 ||
        (student.last_name.toLocaleLowerCase() + ' ' + student.first_name.toLocaleLowerCase()).indexOf(this.searchTerm) !== -1 ||
        (student.first_name.toLocaleLowerCase() + ' ' + student.last_name.toLocaleLowerCase()).indexOf(this.searchTerm) !== -1);
    this.activeList = this.filterList;
  }

  clearFilter() {
    this.searchTerm = '';
    this.filter_active = false;
    this.activeList = this.students;
  }
}