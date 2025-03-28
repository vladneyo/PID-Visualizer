import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { environment } from '../environments/environment';

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
export class AppComponent {
  inputModel: InputModel = {
    target: 1.5,
    timeResponse: 1.0,
    pid: {
      kp: 0.5,
      ki: 0.0,
      kd: 0.0,
    },
    droneModel: 'cetus_pro',
  };

  imageUrl = `${environment.apiUrl}/api/image`;

  constructor(private http: HttpClient) {}

  onSettingsChanged() {
    // Use the environment variable to specify the host
    this.http.post(`${environment.apiUrl}/api/input`, this.inputModel)
      .subscribe(() => {
        this.imageUrl = `${environment.apiUrl}/api/image?t=${new Date().getTime()}`;
      });
  }
}