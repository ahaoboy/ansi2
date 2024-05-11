import { to_svg } from '../sync'

async function readToString() {

  return new Promise<string>((resolve) => {
    const { stdin } = process
    const v = []
    stdin.on('data', (data: Buffer) => {
      v.push(data.toString())
    });

    stdin.on('close', (code) => {
      resolve(v.join(''))
    });
  })


}


async function main() {
  const a = await readToString()
  console.log(a)

  console.log(to_svg(a))
}
main()