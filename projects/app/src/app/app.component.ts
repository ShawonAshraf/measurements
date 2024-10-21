import { ChangeDetectionStrategy, Component, type OnInit, ChangeDetectorRef } from '@angular/core';
import { SamplingProcessor, initSampler } from 'sampler';

// interface similar to the one in the wasm module
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
  styleUrl: './app.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AppComponent implements OnInit {
  sampledMeasurements: Measurement[] = [];
  m: String = 'Sampler Component';
  
  errorMessage: string = '';

  private processor: SamplingProcessor | null = null;

  constructor(private cd: ChangeDetectorRef) {}

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
    if (!file || !this.processor) return;

    try {
      const text = await file.text();
      const measurements = this.parseMeasurements(text);

      // add data
      measurements.forEach((m) => {
        this.processor!.add_measurement(
          m.timestamp,
          m.measurement_type,
          m.value
        );
      });

      // update UI with processed or sampled data
      const sampledData = this.processor.process_measurements();
      this.sampledMeasurements = JSON.parse(sampledData);
      this.errorMessage = '';

      // render the table immediately
      this.cd.detectChanges();
    } catch (error) {
      this.errorMessage = 'Error processing file';
      console.error(error);
    }
  }

  // the wasm returns json data, so we need to parse it
  private parseMeasurements(text: string): Measurement[] {
    return text
      .split('\n')
      .filter((line) => line.trim())
      .map((line) => {
        // remove curly braces and split by comma
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


