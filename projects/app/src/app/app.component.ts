import { ChangeDetectionStrategy, Component, type OnInit } from '@angular/core';
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
  styleUrl: './app.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush,

})
export class AppComponent implements OnInit {
  sampledMeasurements: Measurement[] = [];
  m: String = "Sampler Component";
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
      console.log(this.sampledMeasurements);

      return this.sampledMeasurements;
    } catch (error) {
      this.errorMessage = 'Error processing file';
      console.error(error);
      return [];
    }
  }

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


