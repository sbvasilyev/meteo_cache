import { sleep, check } from 'k6';
import http from 'k6/http';

export const options = {
  stages: [
    { duration: '1m', target: 200 },
    { duration: '5m', target: 200 },
    { duration: '1m', target: 400 },
    { duration: '5m', target: 400 },
    { duration: '1m', target: 800 },
    { duration: '5m', target: 800 },
    { duration: '1m', target: 1000 },
    { duration: '5m', target: 1000 },
    { duration: '1m', target: 2000 },
    { duration: '5m', target: 2000 },
    { duration: '0m', target: 2000 },
  ],
};

export default () => {
  const res = http.get('http://localhost:8081/forecast?city=Moscow&timezone=auto&forecast_days=3&hourly=temperature_2m');
  check(res, { '200': (r) => r.status === 200 });
  sleep(1);
};
