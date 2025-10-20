import { sleep, check } from 'k6';
import http from 'k6/http';

export const options = {
  stages: [
    { duration: '1m', target: 200 },
    { duration: '5m', target: 200 },
    { duration: '1m', target: 0 },
  ],
  thresholds: {
    http_req_duration: ['p(99)<300']
  },
};

export default () => {
  const res = http.get('http://localhost:8081/forecast?city=Moscow&timezone=auto&forecast_days=3&hourly=temperature_2m');
  check(res, { '200': (r) => r.status === 200 });
  sleep(1);
};
