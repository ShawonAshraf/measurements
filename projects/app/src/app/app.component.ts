import { ChangeDetectionStrategy, Component, type OnInit, signal } from '@angular/core';
import { SamplingProcessor, initSampler } from 'sampler';

interface Measurement {
  timestamp: string;
  measurement_type: string;
  value: number;
}


@Component({
  selector: 'app-root',
  standalone: true,
  imports: [],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AppComponent implements OnInit {
  sampledMeasurements: Measurement[] = [];
  errorMessage: string = '';
  private processor: SamplingProcessor | null = null;

  async ngOnInit() {
    try {
      await initSampler();
      this.processor = new SamplingProcessor();
    } catch (error) {
      this.errorMessage = 'Failed to initialize WASM module';
      console.error(error);
    }
  }

  async onFileSelected(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    console.log(file);
    if (!file || !this.processor) return;

    try {
      const text = await file.text();
      // console.log(text);
      const measurements = this.parseMeasurements(text);
      // console.log(measurements);

      // Add measurements to the WASM processor
      measurements.forEach((m) => {
        this.processor!.add_measurement(
          m.timestamp,
          m.measurement_type,
          m.value
        );
      });

      // Process and update the UI
      // TODO: fix this
      // returning []
      const sampledData = this.processor.process_measurements();
      console.log(sampledData);
      this.sampledMeasurements = JSON.parse(sampledData);
      this.errorMessage = '';
    } catch (error) {
      this.errorMessage = 'Error processing file';
      console.error(error);
    }
  }

  private parseMeasurements(text: string): Measurement[] {
    return text
      .split('\n')
      .filter((line) => line.trim())
      .map((line) => {
        // Remove curly braces and split by comma
        const [timestamp, type, value] = line
          .replace(/{|}/g, '')
          .split(',')
          .map((s) => s.trim());

        return {
          timestamp,
          measurement_type: type,
          value: parseFloat(value),
        };
      });
  }
}


