import initSync, {get_years, get_leadership_data_by_lp_year, get_leadership_data_by_year, get_leadership_labels} from './pkg/at_wasm.js';
async function start(){
    await initSync('./pkg/at_wasm_bg.wasm');
}
await start();
// TODO we have to wait for start to complete it's work before we can move on.
await new Promise(r => setTimeout(r, 1000));

const years = get_years();
const leadership_labels = get_leadership_labels();
for (const year of years) {
    const ctx = document.getElementById('canvas_principles_'+year);
    console.log(ctx)

    const YlGnBu9 = ['#4E79A7', '#A0CBE8', '#F28E2B', '#FFBE7D', '#59A14F', '#8CD17D', '#B6992D', '#F1CE63', '#499894', '#86BCB6', '#E15759', '#FF9D9A', '#79706E', '#BAB0AC', '#D37295', '#FABFD2', '#B07AA1', '#D4A6C8', '#9D7660', '#D7B5A6'];
    const leadership_data = get_leadership_data_by_year(year);
    const myChart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: leadership_labels,
            datasets: [{
                label: '# Leadership Principles',
                data: leadership_data,
                backgroundColor: 'rgba(255, 99, 132, 0.2)',
                borderColor: 'rgba(255, 99, 132, 1)',
                borderWidth: 1
            }]
        },
        options: {
            plugins: {
                legend: {
                labels: {
                    // This more specific font property overrides the global property
                    font: {
                        size: 24,
                    }
                }
            } } ,
            scales: {
                y: {
                    beginAtZero: true
                },

                xAxes: {
                    ticks: {
                        autoSkip: false,
                        maxRotation: 70,
                        minRotation: 70,
                        font: {
                            size: 18,
                        },
                    }
                },
            }
        }
    });

    Chart.defaults.font.size = 18;  
    let data = []
    for (const [index, lp] of leadership_labels.entries()) {
        data.push({
            label: lp,
            data: get_leadership_data_by_lp_year(lp, year),
            backgroundColor: YlGnBu9[index],
            borderColor: YlGnBu9[index],    
        });
    }
    const ctx1 = document.getElementById('canvas_principle_month_' + year);
    const myChart1 = new Chart(ctx1, {
        type: 'line',    
        data: {         
            labels: ["Jan", "Feb", "Mar", "Apr", "May", "June", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
            datasets: data, 
        }
    });

}
