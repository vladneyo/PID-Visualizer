import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { environment } from '../environments/environment';
import { firstValueFrom, Subject } from 'rxjs';
import { debounceTime } from 'rxjs/operators';

interface PIDParams {
  kp: number;
  ki: number;
  kd: number;
}

interface InputModel {
  target: number;
  timeResponse: number;
  pid: PIDParams;
  droneModel: string;
}

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  inputModel!: InputModel;
  imageUrl = `${environment.apiUrl}/api/image`;
  private updateSubject = new Subject<void>();

  constructor(private http: HttpClient) { }

  async ngOnInit(): Promise<void> {
    const defaults = await firstValueFrom(this.http.get<InputModel>(`${environment.apiUrl}/api/defaults`));
    this.inputModel = {
      target: defaults.target,
      timeResponse: defaults.timeResponse,
      pid: defaults.pid,
      droneModel: defaults.droneModel,
    };

    this.updateSubject.pipe(
      debounceTime(500)
    ).subscribe(() => {
      this.http.post(`${environment.apiUrl}/api/input`, this.inputModel)
        .subscribe(() => {
          this.imageUrl = `${environment.apiUrl}/api/image?t=${new Date().getTime()}`;
        });
    });
  }

  onSettingsChanged() {
    this.updateSubject.next();
  }
}